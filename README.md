# whisper-services-rs

pip install tokenizers==0.20.3

az storage blob upload --account-name ygpub --container-name whisper-services-rs --file target/release/whisper-services-rs --name whisper-services-rs --overwrite

az storage blob upload-batch --account-name ygpub -d whisper-services-rs/models/0.17.0/whisper_turbo_int8_a100_beam5_batch8 -s ../whisper-trtllm-rs/models/whisper_turbo_int8_a100_beam5_batch8  --overwrite

wget -P /app/whisper-services-rs/audio https://ygpub.blob.core.windows.net/whisper-services-rs/audio/oppo-en-us.wav

cargo update -p whisper-trtllm-rs

curl https://speech.yglabs.eu.org/v1/audio/detections \
  -H "Content-Type: multipart/form-data" \
  -F file="@/audio/oppo-en-us.wav"
  -v --trace-time

curl http://127.0.0.1:3000/v1/audio/detections \
  -H "Content-Type: multipart/form-data" \
  -F file="@audio/oppo-en-us.wav" \
  -v --trace-time

curl http://127.0.0.1:3000/v1/audio/transcriptions \
  -H "Content-Type: multipart/form-data" \
  -F file="@audio/oppo-zh-ch.wav" \
  -v --trace-time

curl https://speech.yglabs.eu.org/v1/audio/transcriptions \
  -H "Content-Type: multipart/form-data" \
  -F file="@audio/en-us-5mins.wav" \
  -v --trace-time
