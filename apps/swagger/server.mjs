import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const distDir = path.dirname(require.resolve("swagger-ui-dist/package.json"));
const here = path.dirname(fileURLToPath(import.meta.url));
const specPath = process.env.OPENAPI_PATH ?? path.resolve(here, "../api/openapi.yaml");
const port = Number(process.env.PORT ?? 8080);

const contentTypes = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".png": "image/png",
  ".yaml": "application/yaml; charset=utf-8",
  ".yml": "application/yaml; charset=utf-8",
  ".json": "application/json; charset=utf-8",
};

function send(response, status, body, contentType) {
  response.writeHead(status, { "content-type": contentType });
  response.end(body);
}

const server = createServer(async (request, response) => {
  try {
    const { pathname } = new URL(request.url, `http://${request.headers.host}`);

    if (pathname === "/" || pathname === "/index.html") {
      const index = await readFile(path.join(here, "index.html"));
      return send(response, 200, index, contentTypes[".html"]);
    }

    if (pathname === "/openapi.yaml") {
      const spec = await readFile(specPath);
      return send(response, 200, spec, contentTypes[".yaml"]);
    }

    if (pathname.startsWith("/swagger-ui/")) {
      const name = path.basename(pathname);
      const asset = await readFile(path.join(distDir, name));
      const contentType = contentTypes[path.extname(name)] ?? "application/octet-stream";
      return send(response, 200, asset, contentType);
    }

    return send(response, 404, "not found", "text/plain; charset=utf-8");
  } catch (error) {
    return send(response, 500, String(error), "text/plain; charset=utf-8");
  }
});

server.listen(port, () => {
  console.log(`Swagger UI: http://localhost:${port}`);
});
