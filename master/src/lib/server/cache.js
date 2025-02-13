class Cache {
    constructor(ttl = 3600000) { 
        this.cache = new Map();
        this.ttl = ttl;
    }

    set(key, value) {
        this.cache.set(key, {
            value,
            timestamp: Date.now()
        });
    }

    get(key) {
        const data = this.cache.get(key);
        if (!data) return null;
        
        if (Date.now() - data.timestamp > this.ttl) {
            this.cache.delete(key);
            return null;
        }
        
        return data.value;
    }
}

export const versionCache = new Cache(3600000); 