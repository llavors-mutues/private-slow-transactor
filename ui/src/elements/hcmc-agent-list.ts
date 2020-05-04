import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, html, query, property } from 'lit-element';
import { ApolloClient, gql } from 'apollo-boost';

import { Agent } from 'holochain-profiles';
import { Dialog } from '@material/mwc-dialog';
import { ApolloClientModule } from '@uprtcl/graphql';

import { MutualCreditBindings } from '../bindings';
import { GetAllowedCreditors } from '../types';

export class MCAgentList extends moduleConnect(LitElement) {
  @query('#create-offer-dialog')
  createOfferDialog!: Dialog;

  @property({ type: String })
  selectedCreditor: string | undefined = undefined;

  @property({ type: Array })
  agents: Agent[] | undefined = undefined;

  client!: ApolloClient<any>;

  async firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);

    const result = await this.client.query({
      query: gql`
        {
          allAgents {
            id
            username
          }
        }
      `,
    });

    const getAllowedCreditors: GetAllowedCreditors = this.request(
      MutualCreditBindings.ValidAgentFilter
    );

    this.agents = await getAllowedCreditors(this.client);
  }

  renderCreateOffer() {
    return html`<mwc-dialog id="create-offer-dialog">
      <hcmc-create-offer .creditor=${this.selectedCreditor}>
      </hcmc-create-offer>
    </mwc-dialog>`;
  }

  renderAgent(agent: Agent) {
    return html`
      <div class="row" style="align-items: center;">
        <mwc-list-item style="flex: 1;" twoline noninteractive>
          <span>${agent.username}</span>
          <span slot="secondary">${agent.id}</span>
        </mwc-list-item>

        <mwc-button
          label="Offer credits"
          @click=${() => {
            this.selectedCreditor = agent.id;
            this.createOfferDialog.open = true;
          }}
        ></mwc-button>
      </div>
    `;
  }

  render() {
    if (!this.agents)
      return html`<mwc-circular-progress></mwc-circular-progress>`;

    return html`
      ${this.renderCreateOffer()}
      <mwc-list>
        ${this.agents.map(
          (agent, i) => html`${this.renderAgent(agent)}
          ${this.agents && i < this.agents.length - 1
            ? html`<li divider padded role="separator"></li> `
            : html``} `
        )}
      </mwc-list>
    `;
  }
}
