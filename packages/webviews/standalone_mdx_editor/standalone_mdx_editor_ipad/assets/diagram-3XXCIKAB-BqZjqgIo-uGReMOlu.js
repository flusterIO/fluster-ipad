import{t as B}from"./chunk-44GW5IO5-DinN5XLx-DThzNzy7.js";import{a5 as v,a6 as z,ao as I,an as P,a7 as S,a8 as j,H as n,aA as x,aS as F,aa as T,at as W,aB as E,aC as A,E as m}from"./main-CSAj7I-T.js";import{T as D}from"./radar-VG2SY3DT-BO1_0p1Z-xQvZ-BSh.js";import"./min-I0heXDTQ-Bmx98OAS.js";import"./_baseUniq-jNvV-1jU-CJcM8TDR.js";var $={packet:[]},w=structuredClone($),H=E.packet,Y=n(()=>{const t=x({...H,...A().packet});return t.showBits&&(t.paddingY+=10),t},"getConfig"),R=n(()=>w.packet,"getPacket"),L=n(t=>{t.length>0&&w.packet.push(t)},"pushWord"),G=n(()=>{W(),w=structuredClone($)},"clear"),u={pushWord:L,getPacket:R,getConfig:Y,clear:G,setAccTitle:j,getAccTitle:S,setDiagramTitle:P,getDiagramTitle:I,getAccDescription:z,setAccDescription:v},J=1e4,M=n(t=>{B(t,u);let e=-1,o=[],s=1;const{bitsPerRow:l}=u.getConfig();for(let{start:a,end:r,bits:d,label:k}of t.blocks){if(a!==void 0&&r!==void 0&&r<a)throw new Error(`Packet block ${a} - ${r} is invalid. End must be greater than start.`);if(a??=e+1,a!==e+1)throw new Error(`Packet block ${a} - ${r??a} is not contiguous. It should start from ${e+1}.`);if(d===0)throw new Error(`Packet block ${a} is invalid. Cannot have a zero bit field.`);for(r??=a+(d??1)-1,d??=r-a+1,e=r,m.debug(`Packet block ${a} - ${e} with label ${k}`);o.length<=l+1&&u.getPacket().length<J;){const[c,p]=X({start:a,end:r,bits:d,label:k},s,l);if(o.push(c),c.end+1===s*l&&(u.pushWord(o),o=[],s++),!p)break;({start:a,end:r,bits:d,label:k}=p)}}u.pushWord(o)},"populate"),X=n((t,e,o)=>{if(t.start===void 0)throw new Error("start should have been set during first phase");if(t.end===void 0)throw new Error("end should have been set during first phase");if(t.start>t.end)throw new Error(`Block start ${t.start} is greater than block end ${t.end}.`);if(t.end+1<=e*o)return[t,void 0];const s=e*o-1,l=e*o;return[{start:t.start,end:s,label:t.label,bits:s-t.start},{start:l,end:t.end,label:t.label,bits:t.end-l}]},"getNextFittingBlock"),q={parse:n(async t=>{const e=await D("packet",t);m.debug(e),M(e)},"parse")},K=n((t,e,o,s)=>{const l=s.db,a=l.getConfig(),{rowHeight:r,paddingY:d,bitWidth:k,bitsPerRow:c}=a,p=l.getPacket(),i=l.getDiagramTitle(),h=r+d,b=h*(p.length+1)-(i?0:r),g=k*c+2,f=F(e);f.attr("viewbox",`0 0 ${g} ${b}`),T(f,b,g,a.useMaxWidth);for(const[C,y]of p.entries())N(f,y,C,a);f.append("text").text(i).attr("x",g/2).attr("y",b-h/2).attr("dominant-baseline","middle").attr("text-anchor","middle").attr("class","packetTitle")},"draw"),N=n((t,e,o,{rowHeight:s,paddingX:l,paddingY:a,bitWidth:r,bitsPerRow:d,showBits:k})=>{const c=t.append("g"),p=o*(s+a)+a;for(const i of e){const h=i.start%d*r+1,b=(i.end-i.start+1)*r-l;if(c.append("rect").attr("x",h).attr("y",p).attr("width",b).attr("height",s).attr("class","packetBlock"),c.append("text").attr("x",h+b/2).attr("y",p+s/2).attr("class","packetLabel").attr("dominant-baseline","middle").attr("text-anchor","middle").text(i.label),!k)continue;const g=i.end===i.start,f=p-2;c.append("text").attr("x",h+(g?b/2:0)).attr("y",f).attr("class","packetByte start").attr("dominant-baseline","auto").attr("text-anchor",g?"middle":"start").text(i.start),g||c.append("text").attr("x",h+b).attr("y",f).attr("class","packetByte end").attr("dominant-baseline","auto").attr("text-anchor","end").text(i.end)}},"drawWord"),O={draw:K},U={byteFontSize:"10px",startByteColor:"black",endByteColor:"black",labelColor:"black",labelFontSize:"12px",titleColor:"black",titleFontSize:"14px",blockStrokeColor:"black",blockStrokeWidth:"1",blockFillColor:"#efefef"},Q=n(({packet:t}={})=>{const e=x(U,t);return`
	.packetByte {
		font-size: ${e.byteFontSize};
	}
	.packetByte.start {
		fill: ${e.startByteColor};
	}
	.packetByte.end {
		fill: ${e.endByteColor};
	}
	.packetLabel {
		fill: ${e.labelColor};
		font-size: ${e.labelFontSize};
	}
	.packetTitle {
		fill: ${e.titleColor};
		font-size: ${e.titleFontSize};
	}
	.packetBlock {
		stroke: ${e.blockStrokeColor};
		stroke-width: ${e.blockStrokeWidth};
		fill: ${e.blockFillColor};
	}
	`},"styles"),at={parser:q,db:u,renderer:O,styles:Q};export{at as diagram};
