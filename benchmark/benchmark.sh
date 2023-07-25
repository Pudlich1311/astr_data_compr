#! /usr/bin/bash

rm results.txt
touch results.txt

orig_file=$(stat -c %s "./data/Gaiatest.csv")

echo "Starting the benchmark"
echo -e "Results of different compression tools \n" >> results.txt 
echo -e "Size of original file $orig_file \n" >> results.txt 

function compress(){
    tool=$1
    command=$2
    output_file=$3
    echo "Compressing with $tool"
    start=$(date +%s)
    $command &> /dev/null
    end=$(date +%s)
    file_size=$(stat -c %s "./${output_file}")

    size_percentage=$(echo "scale=2 ; 1-($file_size / $orig_file)" | bc)
    flock -x results.txt  echo -e "$tool:
    Time: $(($end-$start)) seconds
    Size after compression: $file_size
    $size_percentage percentage compression\n" >> results.txt   
    echo "$tool compression done"
}

function decompress(){
    tool=$1
    command=$2
    input_file=$3
    output_file=$4
    echo "Decompressing with $tool"
    start=$(date +%s)
    $command &> /dev/null
    end=$(date +%s)

    rm $input_file
    rm $output_file
    flock -x results.txt  echo -e "$tool:
    Time: $(($end-$start)) seconds\n\n" >> results.txt 
}

compress "bsc_m03" "./bsc_m03 e ./data/Gaiatest.csv ./comp.bsc -b1000000000" "comp.bsc"

decompress "bsc_m03" "./bsc_m03 d ./comp.bsc ./bsc_output.csv" "comp.bsc" "bsc_output.csv"

compress "7zip" "7z a comp.7z ./data/Gaiatest.csv" "comp.7z"

decompress "7zip" "7z e comp.7z" "comp.7z" "Gaiatest.csv"

compress "zstd" "zstd -z ./data/Gaiatest.csv -o comp.zstd" "comp.zstd"

decompress "zstd" "zstd -d ./comp.zstd -o ./zstd.csv" "comp.zstd" "zstd.csv"

compress "mcm" "mcm -x ./data/Gaiatest.csv ./comp.mcm" "comp.mcm"

decompress "mcm" "mcm d ./comp.mcm ./mcm.csv" "comp.mcm" "mcm.csv"

compress "gzip" "gzip -k -9 ./data/Gaiatest.csv" "./data/Gaiatest.csv.gz"

cp ./data/Gaiatest.csv.gz ./ 
rm ./data/Gaiatest.csv.gz 

decompress "gzip" "gzip -d -k ./Gaiatest.csv.gz" "Gaiatest.csv.gz" "Gaiatest.csv"

compress "my_compressor" "./astr_data_compr -c ./data/Gaiatest.csv" "./data/Gaiatest.comp"

cp ./data/Gaiatest.comp ./
rm ./data/Gaiatest.comp

decompress "my_compressor" "./astr_data_compr -d ./Gaiatest.comp" "Gaiatest.comp" "Gaiatest_decomp.csv"

