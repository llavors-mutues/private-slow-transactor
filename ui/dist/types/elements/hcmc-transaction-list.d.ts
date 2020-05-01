import { LitElement } from 'lit-element';
import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';
import { Transaction } from '../types';
declare const TransactionList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class TransactionList extends TransactionList_base {
    transactions: Array<Transaction>;
    firstUpdated(): Promise<void>;
    render(): import("lit-element").TemplateResult;
}
export {};
