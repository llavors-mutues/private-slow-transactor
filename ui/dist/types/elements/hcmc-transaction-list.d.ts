import { LitElement } from 'lit-element';
import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';
import { Transaction } from '../types';
import { Agent } from 'holochain-profiles';
declare const MCTransactionList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCTransactionList extends MCTransactionList_base {
    myAgentId: string;
    transactions: Array<Transaction>;
    static get styles(): import("lit-element").CSSResult;
    firstUpdated(): Promise<void>;
    isOutgoing(transaction: Transaction): boolean;
    getCounterparty(transaction: Transaction): Agent;
    render(): import("lit-element").TemplateResult;
    renderContent(): import("lit-element").TemplateResult;
}
export {};
