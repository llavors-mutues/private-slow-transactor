import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, html, property, query } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { ApolloClientModule } from '@uprtcl/graphql';
import { CREATE_OFFER } from 'src/graphql/queries';

import '@material/mwc-textfield';
import '@material/mwc-button';
import { TextFieldBase } from '@material/mwc-textfield/mwc-textfield-base';
import { sharedStyles } from './sharedStyles';

export class CreateOffer extends moduleConnect(LitElement) {
  @query('#amount')
  amountField!: TextFieldBase;

  @query('#creditor')
  creditorField!: TextFieldBase;

  @property({ type: String })
  creditor: string | undefined = undefined;

  client!: ApolloClient<any>;

  static get styles() {
    return sharedStyles;
  }

  firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);
  }

  createOffer() {
    this.client.mutate({
      mutation: CREATE_OFFER,
      variables: {
        creditorId: this.creditorField.value,
        amount: parseFloat(this.amountField.value),
      },
    });
  }

  render() {
    return html`
      <div class="column center-content">
        <mwc-textfield
          style="padding: 16px 0;"
          label="Amount"
          type="number"
          id="amount"
          min="0.1"
          step="0.1"
          autoValidate
        ></mwc-textfield>

        <mwc-textfield
          .disabled=${this.creditor !== undefined}
          .value=${this.creditor}
          style="padding-bottom: 16px;"
          id="creditor"
          label="Creditor"
          autoValidate
        ></mwc-textfield>

        <mwc-button
          label="CREATE OFFER"
          raised
          @click=${() => this.createOffer()}
        ></mwc-button>
      </div>
    `;
  }
}
