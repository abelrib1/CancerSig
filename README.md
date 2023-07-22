# CancerSig
Probabilistic engine for cancer outcome prediction.

CancerSig uses multi-nomial data from Cancer patients to predict treatment outcomes.

## Installation on Ubuntu
First, make sure you have the correct libraries
```bash
apt install libssl-dev
sudo apt install cargo
sudo apt install rustc
cargo build --release
```

As well as in Python

```bash
sudo apt-get install openslide-tools
pip install openslide-python
```

Download using your manifest the following way
`
CancerSig/target/release/CancerSig download gdc_manifest.2023-07-05.txt
`

There might be a need to relaunch it from time to time if connections get cut, which you can achieve using the metadata.json file the following way
`
/root/CancerSig/CancerSig/target/release/CancerSig download gdc_manifest.2023-07-05.txt --complete metadata.json 2>&1  | tee log.txt
`

We highly recommend you setting up at least 60GB of SWAP memory.

#
## File numbers
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