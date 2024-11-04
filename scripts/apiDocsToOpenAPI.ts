/// <reference types="node" />
import { z } from "./_helper/zod";
import { type ZodAny, ZodUndefined, objectUtil } from "zod";
import {
	type ValorantEndpoint,
	type EndpointTypeName,
	type EndpointType,
	typeToPrefix,
} from "../api-docs/valorant-api-types/src/ValorantEndpoint";
import { endpoints } from "../api-docs/valorant-api-types/src/endpoints";
import {
	OpenAPIRegistry,
	OpenApiGeneratorV3,
} from "@asteasolutions/zod-to-openapi";
import fs from "node:fs";
import path from "node:path";
import console from "node:console";

async function main() {
	const registerMap = new Map<EndpointTypeName, OpenAPIRegistry>();
	for (const endpoint of Object.values(endpoints) as ValorantEndpoint[]) {
		let registry = registerMap.get(endpoint.type);
		if (!registry) {
			registry = new OpenAPIRegistry();
			registerMap.set(endpoint.type, registry);
		}
		let params: { [key: string]: ZodAny } = {};
		let headers: { [key: string]: ZodAny } = {};
		endpoint.suffix = endpoint.suffix
			.replaceAll(" ", "_")
			.replaceAll("-", "_")
			.replaceAll("https:", "");
		if (endpoint.variables) {
			params = Object.assign(
				params,
				Object.fromEntries(
					Array.from(endpoint.variables.entries()).map(([key, value]) => [
						key.replaceAll(/[{}]/g, "").replace(" ", "_").replace("-", "_"),
						value,
					]),
				),
			);
		}
		if (endpoint.riotRequirements) {
			headers = Object.assign(
				headers,
				Object.fromEntries(
					Object.keys(endpoint.riotRequirements).map((key) => [
						key,
						z.string(),
					]),
				),
			);
		}
		const match = endpoint.suffix.match(/{([a-zA-Z_]*)}/g);
		if (match) {
			params = Object.assign(
				params,
				Object.fromEntries(match.map((key) => [key.slice(1, -1), z.string()])),
			);
		}
		registry.registerPath({
			method: (endpoint.method?.toLowerCase() ?? "get") as "get",
			path: endpoint.type === "other" ? endpoint.suffix : `/${endpoint.suffix}`,
			description: endpoint.description,
			summary: endpoint.name,
			request: {
				headers: z.object(headers),
				params: z.object(params),
			},
			responses: endpoint.responses
				? Object.fromEntries(
						Object.entries(endpoint.responses).map(([key, value]) => [
							key,
							{
								description: "",
								content: {
									"application/json": {
										schema: value instanceof ZodUndefined ? z.null() : value,
									},
								},
							},
						]),
					)
				: {
						200: {
							description: "",
							content: {
								"application/json": {
									schema: z.object({ hoge: z.number() }),
								},
							},
						},
					},
		});
	}
	for (const [type, registry] of registerMap.entries()) {
		const generator = new OpenApiGeneratorV3(registry.definitions);
		const doc = generator.generateDocument({
			openapi: "3.0.0",
			servers: [
				{
					url: typeToPrefix({
						type,
						shard: "{shard}",
						region: "{region}",
						port: "{port}",
					} as EndpointType),
				},
			],
			info: {
				title: "Valorant API",
				version: "1.0.0",
				description: "Valorant API",
			},
		});
		fs.writeFileSync(
			path.join(__dirname, `../openapi/valorant-${type}.json`),
			JSON.stringify(doc, null, 2),
		);
	}
}

main().catch((e) => {
	console.error(e);
	process.exit(1);
});
