import {EventEmitter} from "../libs/events.ts";

Deno.test("Hello world John", () => {
    const eventMng = new EventEmitter();
    eventMng.on('on_test', (name: string) => {
        if (name !== 'John') {
            throw new Error('name should be equal to John')
        }
        return true;
    })
    
    eventMng.emit('on_test', 'John')
})