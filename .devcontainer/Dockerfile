FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

COPY --from=ghcr.io/jdx/mise:latest /usr/local/bin/mise /usr/bin/mise

RUN echo 'eval "$(mise activate bash)"' >> $HOME/.bashrc && \
  echo 'eval "$(mise activate zsh)"' >> $HOME/.zshrc
