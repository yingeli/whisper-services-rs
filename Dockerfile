FROM ygsea.azurecr.io/nvidia/trtllm:0.17.0-80

RUN apt-get update && apt-get install -y \
    #libopenmpi-dev \
    wget

WORKDIR /app/whisper-services-rs

# download engines
RUN mkdir -p models/turbo/encoder && \
    mkdir -p models/turbo/decoder && \
    wget -P models/turbo https://raw.githubusercontent.com/openai/whisper/main/whisper/assets/mel_filters.npz && \
    wget -P models/turbo https://huggingface.co/mobiuslabsgmbh/faster-whisper-large-v3-turbo/resolve/main/tokenizer.json && \
    wget -P  models/turbo/encoder https://ygpub.blob.core.windows.net/whisper-services-rs/models/0.17.0/whisper_turbo_int8_a100_beam5_batch8/encoder/config.json && \
    wget -P models/turbo/encoder https://ygpub.blob.core.windows.net/whisper-services-rs/models/0.17.0/whisper_turbo_int8_a100_beam5_batch8/encoder/rank0.engine && \
    wget -P models/turbo/decoder https://ygpub.blob.core.windows.net/whisper-services-rs/models/0.17.0/whisper_turbo_int8_a100_beam5_batch8/decoder/config.json && \
    wget -P models/turbo/decoder https://ygpub.blob.core.windows.net/whisper-services-rs/models/0.17.0/whisper_turbo_int8_a100_beam5_batch8/decoder/rank0.engine && \
    wget https://ygpub.blob.core.windows.net/whisper-services-rs/whisper-services-rs && \
    chmod +x whisper-services-rs

#ENV LD_LIBRARY_PATH=/opt/hpcx/ucx/lib:/usr/local/lib/python3.12/dist-packages/torch/lib:${LD_LIBRARY_PATH} \
#    OPAL_PREFIX=/opt/hpcx/ompi

ENV LD_LIBRARY_PATH=/usr/local/lib/python3.12/dist-packages/torch/lib:${LD_LIBRARY_PATH}

EXPOSE 3000

CMD ["./whisper-services-rs"]
