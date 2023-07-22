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
Recommend high SWAP memory (60GB)


IDAT
`
download and install from https://support.illumina.com/downloads/iaap-genotyping-cli.html
tar -xvf iaap-cli-linux-x64-1.1.0-sha.80d7e5b3d9c1fdfc2e99b472a90652fd3848bbc7.tar
cd iaap-cli-linux-x64-1.1.0-sha.80d7e5b3d9c1fdfc2e99b472a90652fd3848bbc7/iaap-cli
echo "export PATH=$PATH:$(pwd)" >> ~/.bash_profile
source ~/.bash_profile


copy number variation
73,639

transcriptome profiling
56,480

biospecimen
56,349

dna methylation
48,492

clinical
24,549

simple nucleotide variation
16,133

proteome profiling
7,906






simple nucleotide variation
358,021

sequencing reads
134,282

copy number variation
130,094

structural variation
83,899

transcriptome profiling
79,358

biospecimen
56,451

dna methylation
48,492

clinical
24,660

somatic structural variation
8,349

proteome profiling
7,906