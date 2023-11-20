import { vote_backend } from "../../declarations/vote_backend";

const getCurrentProposal = async (count) => {
    const getCurrentProposal = await vote_backend.get_proposal(
        Number(count)
    );
    setCurrentProposal(getCurrentProposal);
};

await vote_backend.get_proposal_count();