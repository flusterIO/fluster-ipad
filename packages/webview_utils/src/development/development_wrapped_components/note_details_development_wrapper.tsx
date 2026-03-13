import { NoteDetailSheet } from '#/mdx/components/note_details/note_detail_sheet'
import { NoteDetailActions, type SetNoteDetailsAction } from '@/code_gen/typeshare/fluster_core_utilities'
import React, { useEffect, type ReactNode } from 'react'


export const NoteDetailsDevelopmentWrapper = (): ReactNode => {

    useEffect(() => {
        window.handleSwiftAction({
            type: NoteDetailActions.SetNoteDetails,
            payload: JSON.parse("{\"summary\":null,\"topic\":\"Fluster-Sandbox\",\"subject\":\"Testing\",\"last_read_string\":\"30 minutes ago\",\"last_modified_string\":\"30 minutes ago\",\"title\":\"Paper Order\",\"note_id\":\"37E37F51-0EC0-4FED-B55E-3CC5E1DB3FEB\",\"tags\":[],\"citations\":[{\"citation_key\":\"https://doi.org/10.48550/arxiv.2102.12084\",\"html\":\"Singal, A. K. (2021,). <span style=\\\"font-style: italic;\\\">Our Peculiar Motion Inferred from Number Counts of Mid Infra Red AGNs and the Discordance Seen with the Cosmological Principle</span>. arXiv. https://doi.org/<a href=\\\"https://doi.org/10.48550/ARXIV.2102.12084\\\">10.48550/ARXIV.2102.12084</a>\",\"url\":\"https://arxiv.org/abs/2102.12084\"},{\"citation_key\":\"General_Relativity\",\"html\":\"Einstein, Albert. (1916). Die Grundlage der allgemeinen Relativitätstheorie. <span style=\\\"font-style: italic;\\\">Annalen der Physik</span>, <span style=\\\"font-style: italic;\\\">354</span>(7), 769–822.\"},{\"citation_key\":\"Bonvin_2006b\",\"html\":\"Bonvin, C., Durrer, R., & Kunz, M. (2006). Dipole of the Luminosity Distance: A Direct Measure of H(z). <span style=\\\"font-style: italic;\\\">Physical Review Letters</span>, <span style=\\\"font-style: italic;\\\">96</span>(19). https://doi.org/<a href=\\\"https://doi.org/10.1103/physrevlett.96.191302\\\">10.1103/physrevlett.96.191302</a>\",\"url\":\"https://doi.org/10.1103%2Fphysrevlett.96.191302\"},{\"citation_key\":\"Jha_2007\",\"html\":\"Jha, S., Riess, A. G., & Kirshner, R. P. (2007). Improved Distances to Type Ia Supernovae with Multicolor Light-Curve Shapes: MLCS2k2. <span style=\\\"font-style: italic;\\\">The Astrophysical Journal</span>, <span style=\\\"font-style: italic;\\\">659</span>(1), 122–148. https://doi.org/<a href=\\\"https://doi.org/10.1086/512054\\\">10.1086/512054</a>\",\"url\":\"https://doi.org/10.1086/512054\"},{\"citation_key\":\"Michelson_Morely\",\"html\":\"Albert Abraham Michelson, Edward Morley. (1887). On the Relative Motion of the Earth and the Luminiferous Ether. <span style=\\\"font-style: italic;\\\">American Journal of Science</span>, <span style=\\\"font-style: italic;\\\">34</span>(203), 333–345.\"},{\"citation_key\":\"michelson_relative_1887\",\"html\":\"Michelson, A. A., & Morley, E. W. (1887). On the relative motion of the Earth and the luminiferous ether. <span style=\\\"font-style: italic;\\\">American Journal of Science</span>, (203), 333. https://doi.org/<a href=\\\"https://doi.org/10.2475/ajs.s3-34.203.333\\\">10.2475/ajs.s3-34.203.333</a>\",\"url\":\"http://www.ajsonline.org/content/s3-34/203/333.abstract\"},{\"citation_key\":\"einstein1905emc2\",\"html\":\"Einstein, A. (1905). Does the inertia of a body depend upon its energy-content. <span style=\\\"font-style: italic;\\\">Annalen Der Physik</span>, <span style=\\\"font-style: italic;\\\">18</span>(13), 639–641.\"},{\"citation_key\":\"singal_largePV\",\"html\":\"Singal, A. K. (2011). Large peculiar motion of the solar system from the dipole anisotropy in sky brightness due to distant radio sources. <span style=\\\"font-style: italic;\\\">Astrophysical Journal Letters</span>, <span style=\\\"font-style: italic;\\\">742</span>(2), 4. <a href=\\\"http://inis.iaea.org/search/search.aspx?orig_q=RN:44006807\\\">http://inis.iaea.org/search/search.aspx?orig_q=RN:44006807</a>\",\"url\":\"http://inis.iaea.org/search/search.aspx?orig_q=RN:44006807\"}]}") as SetNoteDetailsAction["payload"]

        })
    }, [])
    return (
        <div className="w-full h-full flex flex-col min-h-screen justify-center items-center">
            <NoteDetailSheet />
        </div>
    )
}


NoteDetailsDevelopmentWrapper.displayName = "NoteDetailsDevelopmentWrapper"
