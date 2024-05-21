// Copyright 2024, The Horizen Foundation

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#[allow(dead_code)]
pub static VALID_PROOF: crate::Proof = hex_literal::hex!(
    "
        283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a
        2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d60
        0f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd805
        19e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d8
        0b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed6
        1cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac63
        1a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f03
        1ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0
        143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e
        141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b98710
        0abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b00
        1718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f80
        08a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b9010
        1803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3
        009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a
        04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3
        246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b41
        1c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740
        098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa9
        21121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff4
        11b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a7149
        20e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e924
        0ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf
        0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d7
        0d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9
"
);

#[allow(dead_code)]
pub static VALID_HASH: sp_core::H256 = sp_core::H256(hex_literal::hex!(
    "337d23faf65147cd3a2cc495aac5cfbe44fe55b17c83990f43f3e5663b0ff248"
));
#[allow(dead_code)]
pub static VALID_HASH_WITH_VK: sp_core::H256 = sp_core::H256(hex_literal::hex!(
    "99dccad77ed9bf61410f649b9358603a91a603ae51b28dd76db4d1410b186b58"
));
#[allow(dead_code)]
pub static DEFAULT_VK_HASH: sp_core::H256 = sp_core::H256(hex_literal::hex!(
    "6fc9745758412a2fd89c21be8542239c860a076620d44a8c7ee933539c0581f7"
));

#[allow(dead_code)]
fn other_vk() -> (fflonk_verifier::VerificationKey, sp_core::H256) {
    (
        serde_json::from_str(
            r#"
        {
            "protocol": "fflonk",
            "curve": "bn128",
            "nPublic": 1,
            "power": 8,
            "k1": "2",
            "k2": "3",
            "w": "3478517300119284901893091970156912948790432420133812234316178878452092729974",
            "w3": "21888242871839275217838484774961031246154997185409878258781734729429964517155",
            "w4": "21888242871839275217838484774961031246007050428528088939761107053157389710902",
            "w8": "19540430494807482326159819597004422086093766032135589407132600596362845576832",
            "wr": "3934201997113527301378493790887117043297977683138175158026010558961874847407",
            "X_2": [
            [
            "18029695676650738226693292988307914797657423701064905010927197838374790804409",
            "14583779054894525174450323658765874724019480979794335525732096752006891875705"
            ],
            [
            "2140229616977736810657479771656733941598412651537078903776637920509952744750",
            "11474861747383700316476719153975578001603231366361248090558603872215261634898"
            ],
            [
            "1",
            "0"
            ]
            ],
            "C0": [
            "11542342760076629271055423140504027081105481381456687208035580820299274289807",
            "4515654398511679460717994398421754391839000188428897061830765916152718260236",
            "1"
            ]
        }
    "#,
        )
        .unwrap(),
        sp_core::H256(hex_literal::hex!(
            "a3ea24b827d4e9735fe06a69ea9a58025b50d7c06ce2ca04e26c778a655fdc3c"
        )),
    )
}
