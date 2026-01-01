// Clean TypeScript code without slop
interface Config {
    requiredFields: string[];
    transformOptions?: TransformOptions;
}

interface TransformOptions {
    uppercase: boolean;
    trimWhitespace: boolean;
}

interface Record {
    id: string;
    name?: string;
    [key: string]: unknown;
}

interface ProcessedRecord {
    id: string;
    name: string;
    processed: boolean;
}

function calculateFibonacci(n: number): number {
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

function mergeSortedArrays<T>(arr1: T[], arr2: T[]): T[] {
    const result: T[] = [];
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
    private config: Config;
    private processedCount: number = 0;

    constructor(config: Config) {
        this.config = config;
    }

    public process(records: Record[]): ProcessedRecord[] {
        return records
            .filter(record => this.isValid(record))
            .map(record => {
                this.processedCount++;
                return this.transform(record);
            });
    }

    public getProcessedCount(): number {
        return this.processedCount;
    }

    private isValid(record: Record): boolean {
        return this.config.requiredFields.every(field => field in record);
    }

    private transform(record: Record): ProcessedRecord {
        return {
            id: record.id,
            name: (record.name ?? '').toUpperCase(),
            processed: true,
        };
    }
}

export { calculateFibonacci, mergeSortedArrays, DataProcessor };
