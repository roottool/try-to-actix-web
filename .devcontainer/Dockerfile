# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.245.2/containers/rust/.devcontainer/base.Dockerfile
ARG VARIANT=${VARIANT:-"bullseye"}
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

ARG USERNAME=${USERNAME:-"vscode"}
USER ${USERNAME}

ARG WORKING_DIR=${WORKING_DIR:-"/workspace"}
WORKDIR ${WORKING_DIR}
