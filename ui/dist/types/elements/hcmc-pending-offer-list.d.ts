import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';
import { Offer } from 'src/types';
declare const MCPendingOfferList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCPendingOfferList extends MCPendingOfferList_base {
    client: ApolloClient<any>;
    offers: Offer[];
    static get styles(): import("lit-element").CSSResult;
    firstUpdated(): Promise<void>;
    offerSelected(transactionId: string): void;
    getPendingOffers(): Offer[];
    render(): import("lit-element").TemplateResult;
}
export {};
