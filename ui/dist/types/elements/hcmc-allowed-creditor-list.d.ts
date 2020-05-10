import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { Agent } from 'holochain-profiles';
import { MCCreateOffer } from './hcmc-create-offer';
declare const MCAllowedCreditorList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCAllowedCreditorList extends MCAllowedCreditorList_base {
    createOfferDialog: MCCreateOffer;
    selectedCreditor: Agent | undefined;
    agents: Agent[] | undefined;
    client: ApolloClient<any>;
    static get styles(): import("lit-element").CSSResult;
    firstUpdated(): Promise<void>;
    renderCreateOffer(): import("lit-element").TemplateResult;
    renderAgent(agent: Agent): import("lit-element").TemplateResult;
    render(): import("lit-element").TemplateResult;
    renderContent(): import("lit-element").TemplateResult;
}
export {};
