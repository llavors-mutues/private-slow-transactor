import { LitElement, PropertyValues } from 'lit-element';
import { Offer } from 'src/types';
import { ApolloClient } from 'apollo-boost';
import { Agent } from 'holochain-profiles';
declare const MCOfferDetail_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCOfferDetail extends MCOfferDetail_base {
    transactionId: string;
    myAgentId: string;
    offer: Offer;
    accepting: boolean;
    client: ApolloClient<any>;
    static get styles(): import("lit-element").CSSResult;
    updated(changedValues: PropertyValues): void;
    loadOffer(): Promise<void>;
    acceptOffer(): void;
    isOutgoing(): boolean;
    getCounterparty(): Agent;
    renderCounterparty(): import("lit-element").TemplateResult;
    render(): import("lit-element").TemplateResult;
}
export {};
