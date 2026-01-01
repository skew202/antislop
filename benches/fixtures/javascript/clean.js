// Clean JavaScript code without slop
function calculateFibonacci(n) {
    if (n <= 1) return n;

    let prev = 0;
    let curr = 1;

    for (let i = 2; i <= n; i++) {
        const next = prev + curr;
        prev = curr;
        curr = next;
    }

    return curr;
}

function mergeSortedArrays(arr1, arr2) {
    const result = [];
    let i = 0;
    let j = 0;

    while (i < arr1.length && j < arr2.length) {
        if (arr1[i] <= arr2[j]) {
            result.push(arr1[i++]);
        } else {
            result.push(arr2[j++]);
        }
    }

    return result.concat(arr1.slice(i), arr2.slice(j));
}

class DataProcessor {
    constructor(config) {
        this.config = config;
        this.processedCount = 0;
    }

    process(records) {
        return records
            .filter(record => this.isValid(record))
            .map(record => {
                this.processedCount++;
                return this.transform(record);
            });
    }

    isValid(record) {
        const requiredFields = this.config.requiredFields || [];
        return requiredFields.every(field => field in record);
    }

    transform(record) {
        return {
            id: record.id,
            name: (record.name || '').toUpperCase(),
            processed: true,
        };
    }
}

module.exports = { calculateFibonacci, mergeSortedArrays, DataProcessor };
