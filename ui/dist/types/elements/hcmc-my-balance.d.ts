import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@authentic/mwc-circular-progress';
declare const MyBalance_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MyBalance extends MyBalance_base {
    client: ApolloClient<any>;
    balance: number;
    firstUpdated(): Promise<void>;
    render(): import("lit-element").TemplateResult;
}
export {};
