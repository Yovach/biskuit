type Callback = (...args: unknown[]) => boolean;

export class EventManager {

    private listener: Map<string, Callback[]> = new Map();


    public on(eventName: string, callback: Callback) {
        if (!this.listener.has(eventName)) {
            this.listener.set(eventName, []);
        }
        this.listener.get(eventName)?.push(callback);
    }

    public emit(eventName: string, ...args: unknown[]) {
        if (this.listener.has(eventName)) {
            this.listener.get(eventName)?.forEach((callback) => {
                callback(args)
            })
        }
    }
}