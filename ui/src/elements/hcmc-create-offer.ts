import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, html, property, query } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { ApolloClientModule } from '@uprtcl/graphql';
import { CREATE_OFFER } from 'src/graphql/queries';

import '@material/mwc-textfield';
import '@material/mwc-button';
import { TextFieldBase } from '@material/mwc-textfield/mwc-textfield-base';
import { sharedStyles } from './sharedStyles';

export class MCCreateOffer extends moduleConnect(LitElement) {
  @query('#amount')
  amountField!: TextFieldBase;

  @query('#creditor')
  creditorField!: TextFieldBase;

  @property({ type: Boolean })
  open: boolean = false;

  @property({ type: String })
  creditor: string | undefined = undefined;

  client!: ApolloClient<any>;

  static get styles() {
    return sharedStyles;
  }

  firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);
  }

  async createOffer() {
    const creditorId = this.creditorField.value;
    const amount = parseFloat(this.amountField.value);
    await this.client.mutate({
      mutation: CREATE_OFFER,
      variables: {
        creditorId,
        amount,
      },
    });

    this.dispatchEvent(
      new CustomEvent('offer-created', {
        detail: { creditorId, amount },
        composed: true,
        bubbles: true,
      })
    );
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
