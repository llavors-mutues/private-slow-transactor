import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { Agent } from 'holochain-profiles';
import { Dialog } from '@material/mwc-dialog';
declare const MCAgentList_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCAgentList extends MCAgentList_base {
    createOfferDialog: Dialog;
    selectedCreditor: string | undefined;
    agents: Agent[] | undefined;
    client: ApolloClient<any>;
    firstUpdated(): Promise<void>;
    renderCreateOffer(): import("lit-element").TemplateResult;
    renderAgent(agent: Agent): import("lit-element").TemplateResult;
    render(): import("lit-element").TemplateResult;
}
export {};
