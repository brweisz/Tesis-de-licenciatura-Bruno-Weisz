#!/bin/bash

# Number of runs
runs=50

# Helper function to run the benchmark and aggregate the results
benchmark() {
    local program_name=$1
    local command=$2

    # Initialize variables to hold total, min, and max times
    total_time=0
    min_time=999999
    max_time=0

    echo "Benchmarking $program_name:"
    for i in $(seq 1 $runs); do
        echo "Run $i:"

        # Capture the real time output
        real_time=$( { /usr/bin/time -f "%e" $command > /dev/null; } 2>&1 )

        # Convert time to integer for easier aggregation
        time_in_ms=$(echo "$real_time * 1000" | bc | cut -d'.' -f1)

        # Add to total time
        total_time=$((total_time + time_in_ms))

        # Check for min and max
        if (( time_in_ms < min_time )); then
            min_time=$time_in_ms
        fi
        if (( time_in_ms > max_time )); then
            max_time=$time_in_ms
        fi

        echo "Time for run $i: $real_time seconds"
        echo ""
    done

    # Calculate average time
    avg_time=$((total_time / runs))

    echo "$program_name Results:"
    echo "Average time: $(echo "$avg_time / 1000" | bc).$(echo "$avg_time % 1000" | bc) seconds"
    echo "Minimum time: $(echo "$min_time / 1000" | bc).$(echo "$min_time % 1000" | bc) seconds"
    echo "Maximum time: $(echo "$max_time / 1000" | bc).$(echo "$max_time % 1000" | bc) seconds"
    echo ""
}

# Run the benchmarks
benchmark "bb" "bb prove -b ./target/assert_zero.json -w ./target/witness_bb.gz -o ./target/proof"
benchmark "Noirky2" "../../../target/release/plonky2-backend prove -b ./target/assert_zero_noirky2.json -w ./target/witness_noirky2.gz -o ./target/proof_noirky2"
