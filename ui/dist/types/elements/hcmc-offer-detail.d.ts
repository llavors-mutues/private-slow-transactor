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
    consenting: boolean;
    canceling: boolean;
    client: ApolloClient<any>;
    static get styles(): import("lit-element").CSSResult;
    updated(changedValues: PropertyValues): void;
    loadOffer(): Promise<void>;
    acceptOffer(): null | undefined;
    consentOffer(): void;
    cancelOffer(): Promise<void>;
    isOutgoing(): boolean;
    getCounterparty(): Agent;
    getExecutableStatus(): string;
    getCounterpartyUsername(): string;
    userShouldWait(): boolean | undefined;
    renderCounterpartyStatus(): import("lit-element").TemplateResult | undefined;
    renderCounterparty(): import("lit-element").TemplateResult;
    placeholderMessage(): "Accepting offer..." | "Canceling offer..." | "Consenting for offer..." | "Fetching and verifying counterparty chain...";
    getForwardActionLabel(): "Awaiting for agent to be online" | "Awaiting for consent" | "Awaiting for approval";
    renderOfferForwardAction(): import("lit-element").TemplateResult;
    render(): import("lit-element").TemplateResult;
}
export {};
