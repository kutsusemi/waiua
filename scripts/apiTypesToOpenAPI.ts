import console from "node:console";
import fs from "node:fs";
import path from "node:path";
import {
	OpenAPIRegistry,
	OpenApiGeneratorV3,
} from "@asteasolutions/zod-to-openapi";
import type { SecurityRequirementObject } from "openapi3-ts/oas30";
import { type ZodAny, ZodUndefined, objectUtil } from "zod";
import {
	type EndpointType,
	type EndpointTypeName,
	type ValorantEndpoint,
	typeToPrefix,
} from "../api-docs/valorant-api-types/src/ValorantEndpoint";
import { endpoints } from "../api-docs/valorant-api-types/src/endpoints";
/// <reference types="node" />
import { z } from "./_helper/zod";

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
		const security: SecurityRequirementObject[] = [];
		endpoint.suffix = endpoint.suffix.replaceAll("https:", "");
		if (endpoint.variables) {
			params = Object.assign(
				params,
				Object.fromEntries(
					Array.from(endpoint.variables.entries()).map(([key, value]) => [
						key
							.replaceAll(/[{}]/g, "")
							.replaceAll(" ", "_")
							.replaceAll("-", "_"),
						value,
					]),
				),
			);
		}
		if (endpoint.riotRequirements) {
			if (endpoint.riotRequirements.localAuth) {
				security.push({ basicAuth: [] });
			}
			if (endpoint.riotRequirements.token) {
				security.push({ bearerAuth: [] });
			}
			if (endpoint.riotRequirements.entitlement) {
				headers = Object.assign(headers, {
					"X-Riot-Entitlements-JWT": z.string(),
				});
			}
			if (endpoint.riotRequirements.clientVersion) {
				headers = Object.assign(headers, {
					"X-Riot-ClientVersion": z.string(),
				});
			}
			if (endpoint.riotRequirements.clientPlatform) {
				headers = Object.assign(headers, {
					"X-Riot-ClientPlatform": z.string(),
				});
			}
		}
		const match = endpoint.suffix.match(/{([a-zA-Z_\s-]*)}/g);
		if (match) {
			for (const key of match) {
				const sanitizedKey = key.replaceAll(" ", "_").replaceAll("-", "_");
				endpoint.suffix = endpoint.suffix.replace(key, sanitizedKey);
				params = Object.assign(params, {
					[sanitizedKey.slice(1, -1)]: z.string(),
				});
			}
		}
		registry.registerComponent("securitySchemes", "basicAuth", {
			type: "http",
			scheme: "basic",
		});
		registry.registerComponent("securitySchemes", "bearerAuth", {
			type: "http",
			scheme: "bearer",
			bearerFormat: "JWT",
		});
		registry.registerPath({
			method: (endpoint.method?.toLowerCase() ?? "get") as "get",
			path: endpoint.type === "other" ? endpoint.suffix : `/${endpoint.suffix}`,
			description: endpoint.description,
			summary: endpoint.name,
			security: security,
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
