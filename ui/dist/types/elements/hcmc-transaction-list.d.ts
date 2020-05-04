import { LitElement } from 'lit-element';
import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';
import { Transaction } from '../types';
declare const MCTransactionList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCTransactionList extends MCTransactionList_base {
    transactions: Array<Transaction>;
    firstUpdated(): Promise<void>;
    render(): import("lit-element").TemplateResult;
}
export {};
