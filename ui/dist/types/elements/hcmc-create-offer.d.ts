import { LitElement } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@material/mwc-textfield';
import '@material/mwc-button';
import { TextFieldBase } from '@material/mwc-textfield/mwc-textfield-base';
declare const CreateOffer_base: {
    new (...args: any[]): import("@uprtcl/micro-orchestrator").ConnectedElement;
    prototype: any;
} & typeof LitElement;
export declare class CreateOffer extends CreateOffer_base {
    amountField: TextFieldBase;
    creditorField: TextFieldBase;
    client: ApolloClient<any>;
    static get styles(): import("lit-element").CSSResult;
    firstUpdated(): void;
    createOffer(): void;
    render(): import("lit-element").TemplateResult;
}
export {};
