Just me learning a bunch of stuff

## Jupyter Notebooks

The `Dockerfile` defines a Jupyter environment based on the `jupyter/datascience-notebook` image; start it with `./scripts/jupyter`. The notebook workspace root is the repository root so each notebook can be stored alongside the topic it relates to.

Edit `requirements.txt` to configure additional packages to be installed with `pip`; restart the container to take effect.

The interface is hosted at `127.0.0.1:8888` - a URL is printed at container startup including the auth token.