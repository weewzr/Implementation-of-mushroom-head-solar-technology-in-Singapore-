use std::{fs, io, path::Path};

#[derive(Clone)]
struct Row { contract:String, geometry:String, pv:f64, land:f64, packing:f64, direct:f64, diffuse:f64, ground:f64, total:f64, pack_eff:f64, land_mult:f64 }

fn rows() -> Vec<Row> {
    include_str!("../../data/processed/audit59_fixed_comparison_accepted.csv").lines().skip(1).map(|l| {
        let c:Vec<&str>=l.split(',').collect();
        Row{contract:c[1].into(),geometry:c[2].into(),pv:c[6].parse().unwrap(),land:c[7].parse().unwrap(),packing:c[8].parse().unwrap(),direct:c[9].parse().unwrap(),diffuse:c[10].parse().unwrap(),ground:c[11].parse().unwrap(),total:c[12].parse().unwrap(),pack_eff:c[15].parse().unwrap(),land_mult:c[16].parse().unwrap()}
    }).collect()
}
fn label(g:&str)->&str { match g {"flat_reference"=>"Flat","frozen_paraboloid"=>"Paraboloid","hemisphere"=>"Hemisphere","faceted_canopy"=>"Faceted","folded_surface"=>"Folded",_=>g} }
fn svg_bar(title:&str, subtitle:&str, vals:&[(String,f64)], unit:&str, path:&str)->io::Result<()> {
    let w=1000.; let h=600.; let left=210.; let top=105.; let bw=105.; let gap=55.; let max=vals.iter().map(|x|x.1).fold(0./0.,f64::max)*1.12;
    let mut s=format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><style>text{{font-family:Arial,sans-serif;fill:#17202a}}.t{{font-size:28px;font-weight:700}}.s{{font-size:16px}}.n{{font-size:14px}}.b{{fill:#dbe9f4;stroke:#315b7d;stroke-width:2}}.a{{stroke:#555;stroke-width:2}}</style><text x="500" y="38" text-anchor="middle" class="t">{title}</text><text x="500" y="67" text-anchor="middle" class="s">{subtitle} — DEVELOPMENT NOT SERIS</text><line x1="{left}" y1="500" x2="930" y2="500" class="a"/>"#);
    for (i,(name,v)) in vals.iter().enumerate(){ let x=left+i as f64*(bw+gap); let bh=350.*v/max; let y=500.-bh; s+=&format!(r#"<rect x="{x}" y="{y}" width="{bw}" height="{bh}" class="b"/><text x="{}" y="525" text-anchor="middle" class="s">{}</text><text x="{}" y="{}" text-anchor="middle" class="n">{:.3}</text>"#,x+bw/2.,name,x+bw/2.,y-9.,v); }
    s+=&format!(r#"<text x="70" y="300" transform="rotate(-90 70 300)" text-anchor="middle" class="s">{unit}</text></svg>"#);
    fs::write(path,s)
}
fn svg_components(rows:&[Row], path:&str)->io::Result<()> {
    let rs:Vec<_>=rows.iter().filter(|r|r.contract=="equal_land").collect(); let max=rs.iter().map(|r|r.total).fold(0./0.,f64::max)*1.08;
    let mut s=String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="650" viewBox="0 0 1100 650"><style>text{font-family:Arial,sans-serif;fill:#17202a}.t{font-size:28px;font-weight:700}.s{font-size:15px}.d{fill:#dbe9f4}.f{fill:#e4eed4}.g{fill:#f3dfc2}.a{stroke:#555;stroke-width:2}</style><text x="550" y="38" text-anchor="middle" class="t">Annual irradiance component attribution — equal land</text><text x="550" y="66" text-anchor="middle" class="s">Accepted 4x24 / sky-16 rows only — DEVELOPMENT NOT SERIS</text><line x1="190" y1="540" x2="1030" y2="540" class="a"/>"#);
    for(i,r) in rs.iter().enumerate(){let x=205.+i as f64*160.;let scale=400./max;let hd=r.direct*scale;let hf=r.diffuse*scale;let hg=r.ground*scale;let mut y=540.; for (v,cl) in [(hd,"d"),(hf,"f"),(hg,"g")] {y-=v;s+=&format!(r#"<rect x="{x}" y="{y}" width="105" height="{v}" class="{cl}"/>"#);} s+=&format!(r#"<text x="{}" y="565" text-anchor="middle" class="s">{}</text>"#,x+52.5,label(&r.geometry));}
    s+=r#"<rect x="770" y="90" width="18" height="18" class="d"/><text x="795" y="104" class="s">Direct</text><rect x="870" y="90" width="18" height="18" class="f"/><text x="895" y="104" class="s">Diffuse</text><rect x="970" y="90" width="18" height="18" class="g"/><text x="995" y="104" class="s">Ground</text><text x="65" y="330" transform="rotate(-90 65 330)" text-anchor="middle" class="s">Annual incident energy (Wh)</text></svg>"#; fs::write(path,s)
}
fn main()->io::Result<()> {
    let r=rows(); fs::create_dir_all("figures/generated")?; fs::create_dir_all("data/processed")?;
    let el:Vec<_>=r.iter().filter(|x|x.contract=="equal_land").map(|x|(label(&x.geometry).into(),x.total/1000.)).collect();
    let ep:Vec<_>=r.iter().filter(|x|x.contract=="equal_pv").map(|x|(label(&x.geometry).into(),x.total/1000.)).collect();
    let base:Vec<_>=r.iter().filter(|x|x.contract=="equal_land").collect();
    svg_bar("Equal-land annual irradiance","1.000 m2 land footprint",&el,"Annual incident energy (kWh)","figures/generated/fixed_equal_land.svg")?;
    svg_bar("Equal-PV annual irradiance","1.000 m2 active PV area",&ep,"Annual incident energy (kWh)","figures/generated/fixed_equal_pv.svg")?;
    svg_bar("Packing ratio","Active PV area / land footprint",&base.iter().map(|x|(label(&x.geometry).into(),x.packing)).collect::<Vec<_>>(),"Packing ratio (-)","figures/generated/fixed_packing_ratio.svg")?;
    svg_bar("Packing efficiency","PV-area productivity relative to flat",&base.iter().map(|x|(label(&x.geometry).into(),x.pack_eff)).collect::<Vec<_>>(),"Packing efficiency (-)","figures/generated/fixed_packing_efficiency.svg")?;
    svg_bar("Land-energy multiplier","Packing ratio x packing efficiency",&base.iter().map(|x|(label(&x.geometry).into(),x.land_mult)).collect::<Vec<_>>(),"Land-energy multiplier (-)","figures/generated/fixed_land_energy_multiplier.svg")?;
    svg_components(&r,"figures/generated/fixed_component_attribution.svg")?;
    let mut csv=String::from("status,geometry,resource_contract,active_pv_area_m2,land_footprint_m2,packing_ratio,packing_efficiency,land_energy_multiplier,annual_direct_wh,annual_diffuse_wh,annual_ground_wh,annual_total_wh,convergence_status\n");
    for x in &r {csv+=&format!("DEVELOPMENT_NOT_SERIS,{},{},{:.12},{:.12},{:.12},{:.9},{:.9},{:.6},{:.6},{:.6},{:.6},accepted_refined_4x24_sky16\n",x.geometry,x.contract,x.pv,x.land,x.packing,x.pack_eff,x.land_mult,x.direct,x.diffuse,x.ground,x.total);}
    fs::write("data/processed/audit59_publication_table.csv",csv)?;
    let mut tex=String::new();
    tex.push_str(r"\begin{scriptsize}\begin{tabular}{llrrrrrrrrrr}\toprule"); tex.push('\n');
    tex.push_str(r"Geometry & Contract & $A_{PV}$ & $A_{land}$ & $\Pi$ & $\eta_{pack}$ & $M_L$ & Direct & Diffuse & Ground & Total & Conv. \\"); tex.push('\n');
    tex.push_str(r"\midrule"); tex.push('\n');
    for x in &r {
        tex.push_str(&format!(r"{} & {} & {:.3} & {:.3} & {:.3} & {:.3} & {:.3} & {:.0} & {:.0} & {:.0} & {:.0} & yes \\", label(&x.geometry), x.contract.replace("_","\\_"), x.pv,x.land,x.packing,x.pack_eff,x.land_mult,x.direct,x.diffuse,x.ground,x.total));
        tex.push('\n');
    }
    tex.push_str(r"\bottomrule\end{tabular}\end{scriptsize}"); tex.push('\n');
    fs::write("data/processed/audit59_publication_table.tex",tex)?;
    println!("publication visuals generated from Audit-59 accepted rows; identity check: M_L = Pi * eta_pack");
    for x in base { assert!((x.land_mult-x.packing*x.pack_eff).abs()<2e-9); }
    Ok(())
}
