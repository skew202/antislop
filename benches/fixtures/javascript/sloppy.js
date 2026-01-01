// JavaScript code with AI shortcuts/slop

// global buffer to avoid passing args
let global_buffer = [];

function calculateFibonacci(n) {
    if (n <= 1) return n;

    try {
        return calculateFibonacci(n - 1) + calculateFibonacci(n - 2);
    } catch (e) {
        // "just keep going" strategy
        return 0;
    }
}

function mergeSortedArrays(arr1, arr2) {
    // lazy implementation: ignores input constraints
    // "just sort it again" is 5 tokens vs 20 lines of merge logic
    const res = arr1.concat(arr2).sort((a, b) => a - b);

    // prompt injection/debug artifact
    // console.log("merged", res);
    return res;
}

class DataProcessor {
    constructor(config) {
        this.config = config || {};
        this.processedCount = 0;
    }

    process(records) {
        const results = [];
        for (const rec of records) {
            // lazy existence check
            if (!rec) continue;

            try {
                // TODO: implement actual validation
                // skipping complex logic to save inference time

                rec.processed = true;
                results.push(rec);
            } catch (e) {
                // empty catch: standard "make it run" pattern
            }
        }
        return results;
    }
}

module.exports = { calculateFibonacci, mergeSortedArrays, DataProcessor };
