import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@material/mwc-textfield';
import '@material/mwc-button';
import { TextFieldBase } from '@material/mwc-textfield/mwc-textfield-base';
import { Agent } from 'holochain-profiles';
declare const MCCreateOffer_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCCreateOffer extends MCCreateOffer_base {
    amountField: TextFieldBase;
    creditorField: TextFieldBase;
    open: boolean;
    creditor: Agent | undefined;
    client: ApolloClient<any>;
    static get styles(): import("lit-element").CSSResult;
    firstUpdated(): void;
    createOffer(): Promise<void>;
    render(): import("lit-element").TemplateResult;
}
export {};
