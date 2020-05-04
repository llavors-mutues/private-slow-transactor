import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@material/mwc-textfield';
import '@material/mwc-button';
import { TextFieldBase } from '@material/mwc-textfield/mwc-textfield-base';
declare const MCCreateOffer_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class MCCreateOffer extends MCCreateOffer_base {
    amountField: TextFieldBase;
    creditorField: TextFieldBase;
    creditor: string | undefined;
    client: ApolloClient<any>;
    static get styles(): import("lit-element").CSSResult;
    firstUpdated(): void;
    createOffer(): void;
    render(): import("lit-element").TemplateResult;
}
export {};
