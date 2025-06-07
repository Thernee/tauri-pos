FROM ubuntu:22.04
RUN apt update && apt install -y curl build-essential libwebkit2gtk-4.1-dev librsvg2-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
CMD ["bash"]