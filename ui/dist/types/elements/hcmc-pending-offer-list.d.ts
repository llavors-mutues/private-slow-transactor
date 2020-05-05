import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';
import { Offer } from 'src/types';
import { Agent } from 'holochain-profiles';
declare const MCPendingOfferList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCPendingOfferList extends MCPendingOfferList_base {
    client: ApolloClient<any>;
    myAgentId: string;
    offers: Offer[];
    static get styles(): import("lit-element").CSSResult[];
    firstUpdated(): Promise<void>;
    renderPlaceholder(type: string): import("lit-element").TemplateResult;
    offerSelected(transactionId: string): void;
    getPendingOffers(): Offer[];
    getOutgoing(): Offer[];
    getIncoming(): Offer[];
    counterparty(offer: Offer): Agent;
    renderOfferList(title: string, offers: Offer[]): import("lit-element").TemplateResult;
    render(): import("lit-element").TemplateResult;
}
export {};
