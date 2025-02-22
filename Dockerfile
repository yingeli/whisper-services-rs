FROM ygsea.azurecr.io/nvidia/trtllm:0.17.0-75

#RUN apt-get update && apt-get install -y \
#    curl \
#    build-essential \
#    git

# install Rust
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
#ENV PATH="/root/.cargo/bin:${PATH}"

# download engines
#RUN mkdir -p /app/whisper-services-rs/models/turbo/encoder && \
#    mkdir -p /app/whisper-services-rs/models/turbo/decoder && \
#    curl -o /app/whisper-services-rs/models/turbo/mel_filters.npz https://raw.githubusercontent.com/openai/whisper/main/whisper/assets/mel_filters.npz && \
#    curl -o /app/whisper-services-rs/models/turbo/tokenizer.json https://huggingface.co/mobiuslabsgmbh/faster-whisper-large-v3-turbo/resolve/main/tokenizer.json && \
#    curl -o /app/whisper-services-rs/models/turbo/encoder/config.json https://ygpublic.blob.core.windows.net/whisper/engines/0.17/whisper_turbo_int8_t4_beam1_batch2/encoder/config.json && \
#    curl -o /app/whisper-services-rs/models/turbo/encoder/rank0.engine https://ygpublic.blob.core.windows.net/whisper/engines/0.17/whisper_turbo_int8_t4_beam1_batch2/encoder/rank0.engine && \
#    curl -o /app/whisper-services-rs/models/turbo/decoder/config.json https://ygpublic.blob.core.windows.net/whisper/engines/0.17/whisper_turbo_int8_t4_beam1_batch2/decoder/config.json && \
#    curl -o /app/whisper-services-rs/models/turbo/decoder/rank0.engine https://ygpublic.blob.core.windows.net/whisper/engines/0.17/whisper_turbo_int8_t4_beam1_batch2/decoder/rank0.engine

#WORKDIR /app
#RUN git clone https://github.com/yingeli/whisper-services-rs.git

#WORKDIR /app/whisper-services-rs
#RUN cargo build --release

#EXPOSE 3000

#CMD ["./target/release/whisper-services-rs"]