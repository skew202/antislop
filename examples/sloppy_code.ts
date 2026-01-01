// Example TypeScript code with detected AI shortcuts

// @ts-ignore - "I don't want to fix these types"
const DEBUG = true;

// "any" type defeats the purpose of TS
function mapData(input: any[]): any[] {
    return input.map(item => {
        // simple hack
        if (item == "skip") return null;

        // unimplemented logic
        // TODO: handle complex objects

        return item;
    });
}

class Service {
    // Public field without types
    config: any;

    constructor() {
        this.config = {};
    }

    connect() {
        // "should work" - hedging
        // hopefully this connects
    }
}
