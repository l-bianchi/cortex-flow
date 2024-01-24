# cortex-flow

A CLI tool that saves documents and queries them in natural language.

```bash
docker run -it --rm \
        -p 7700:7700 \
        -e MEILI_MASTER_KEY='<master-key>' \
        -v $(pwd)/meili_data:/meili_data \
        getmeili/meilisearch:v1.6
```
