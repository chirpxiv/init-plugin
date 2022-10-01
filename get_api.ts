// Dependencies

import { ensureDirSync } from "https://deno.land/std/fs/mod.ts";

import { Octokit } from "https://cdn.skypack.dev/@octokit/rest?dts";

// Constants

const repo = "goatcorp/Dalamud";

const outputDir = "./data";
const outputFile = "services.csv";

// API Code

const octokit = new Octokit();

async function getServices() {
	const results = [];

	const iterator = await octokit.paginate.iterator(octokit.rest.search.code, {
		q: `[PluginInterface]+in:file+language:cs+repo:${repo}`,
		per_page: 100
	});

	for await (const {data:items} of iterator) {
		const pageResult = items
		.map((item: {path:string;}) => {
			// Despite specifying the language in the query, GitHub still returns HTML pages.
			if (!item.path.startsWith("Dalamud/") || !item.path.endsWith(".cs"))
				return null;
			
			const path = item.path.split("/"),
				namespace = path.slice(0, -1).join("."),
				className = path.pop()?.split(".")[0];
			
			return [
				className,
				namespace
			].join(",");
		})
		.filter((item: {}) => item != null);

		results.push(...pageResult);
	}
	
	return results;
}

// File exists

function exists(path: string) {
	try {
		return Deno.lstatSync(path).isFile;
	} catch {
		return false;
	}
}

// Main

async function main() {
	const outputPath = `${outputDir}/${outputFile}`;

	ensureDirSync(outputDir);

	if (exists(outputPath))
		return;

	const content = await getServices();
	Deno.writeTextFileSync(outputPath, content.join("\n"));
}

await main();

Deno.exit();