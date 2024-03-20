# cortex-flow

### MeiliSearch

A CLI tool that saves documents and queries them in natural language.

```bash
docker run -it --rm \
        -p 7700:7700 \
        -e MEILI_MASTER_KEY='<master-key>' \
        -v $(pwd)/meili_data:/meili_data \
        getmeili/meilisearch:latest
```

### Enable Vector Search

You need to enable the vector store feature and the ability to generate embeddings automatically to be able to search using vectors.
Call the following MeiliSearch APIs to enable this features using curl or your favorite method.

```bash
curl \
  -X PATCH 'http://localhost:7700/experimental-features/' \
  -H 'Content-Type: application/json'  \
  -H 'Authorization: Bearer <master-key>' \
  --data-binary '{
    "vectorStore": true
  }'
```

and to generate auto-embeddings I used the `BAAI/bge-base-en-v1.5` model on HuggingFace but you can actually use what you want, check MeiliSearch documentation for details: https://www.meilisearch.com/docs/learn/experimental/vector_search#generate-vector-embeddings

```bash
curl \
  -X PATCH 'http://localhost:7700/indexes/movies/settings' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer <master-key>' \
  --data-binary '{
    "embedders": {
      "default": {
        "source": "huggingFace",
        "model": "BAAI/bge-base-en-v1.5"
      }
    }
  }'
```

### Cortext-Flow help

```bash
Usage: cortex-flow [COMMAND]

Commands:
  search
  index
  clean
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
