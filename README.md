# CancerSig
Probabilistic engine for cancer outcome prediction

## Install Ubuntu
`
apt install libssl-dev
sudo apt install cargo
sudo apt install rustc
cargo build --release
`

`
sudo apt-get install openslide-tools
pip install openslide-python
`

Download
`
/root/CancerSig/CancerSig/target/release/CancerSig download gdc_manifest.2023-07-05.txt
`

There might be a need to relaunch it from time to time
`
/root/CancerSig/CancerSig/target/release/CancerSig download gdc_manifest.2023-07-05.txt --complete metadata.json 2>&1  | tee log.txt
`