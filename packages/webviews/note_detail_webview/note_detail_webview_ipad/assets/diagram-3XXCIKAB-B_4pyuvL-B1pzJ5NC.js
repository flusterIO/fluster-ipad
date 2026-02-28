import{t as v}from"./chunk-44GW5IO5-pcP672wa-h5szciLf.js";import{a5 as B,a6 as P,ao as S,an as z,a7 as W,a8 as F,n,aA as m,aS as T,aa as O,at as E,aB as A,aC as D,v as x}from"./main-Skw3-tlY.js";import{T as R}from"./radar-VG2SY3DT-AvK8AXBj-B_2O0inT.js";import"./min-DAFbEQv1-r09HIdZn.js";import"./_baseUniq-C8WatXBP-B0fxRWUk.js";var $={packet:[]},w=structuredClone($),Y=A.packet,H=n(()=>{const t=m({...Y,...D().packet});return t.showBits&&(t.paddingY+=10),t},"getConfig"),I=n(()=>w.packet,"getPacket"),L=n(t=>{t.length>0&&w.packet.push(t)},"pushWord"),M=n(()=>{E(),w=structuredClone($)},"clear"),u={pushWord:L,getPacket:I,getConfig:H,clear:M,setAccTitle:F,getAccTitle:W,setDiagramTitle:z,getDiagramTitle:S,getAccDescription:P,setAccDescription:B},N=1e4,Q=n(t=>{v(t,u);let e=-1,o=[],s=1;const{bitsPerRow:i}=u.getConfig();for(let{start:a,end:r,bits:d,label:k}of t.blocks){if(a!==void 0&&r!==void 0&&r<a)throw new Error(`Packet block ${a} - ${r} is invalid. End must be greater than start.`);if(a??=e+1,a!==e+1)throw new Error(`Packet block ${a} - ${r??a} is not contiguous. It should start from ${e+1}.`);if(d===0)throw new Error(`Packet block ${a} is invalid. Cannot have a zero bit field.`);for(r??=a+(d??1)-1,d??=r-a+1,e=r,x.debug(`Packet block ${a} - ${e} with label ${k}`);o.length<=i+1&&u.getPacket().length<N;){const[c,p]=V({start:a,end:r,bits:d,label:k},s,i);if(o.push(c),c.end+1===s*i&&(u.pushWord(o),o=[],s++),!p)break;({start:a,end:r,bits:d,label:k}=p)}}u.pushWord(o)},"populate"),V=n((t,e,o)=>{if(t.start===void 0)throw new Error("start should have been set during first phase");if(t.end===void 0)throw new Error("end should have been set during first phase");if(t.start>t.end)throw new Error(`Block start ${t.start} is greater than block end ${t.end}.`);if(t.end+1<=e*o)return[t,void 0];const s=e*o-1,i=e*o;return[{start:t.start,end:s,label:t.label,bits:s-t.start},{start:i,end:t.end,label:t.label,bits:t.end-i}]},"getNextFittingBlock"),X={parse:n(async t=>{const e=await R("packet",t);x.debug(e),Q(e)},"parse")},j=n((t,e,o,s)=>{const i=s.db,a=i.getConfig(),{rowHeight:r,paddingY:d,bitWidth:k,bitsPerRow:c}=a,p=i.getPacket(),l=i.getDiagramTitle(),h=r+d,b=h*(p.length+1)-(l?0:r),g=k*c+2,f=T(e);f.attr("viewbox",`0 0 ${g} ${b}`),O(f,b,g,a.useMaxWidth);for(const[C,y]of p.entries())G(f,y,C,a);f.append("text").text(l).attr("x",g/2).attr("y",b-h/2).attr("dominant-baseline","middle").attr("text-anchor","middle").attr("class","packetTitle")},"draw"),G=n((t,e,o,{rowHeight:s,paddingX:i,paddingY:a,bitWidth:r,bitsPerRow:d,showBits:k})=>{const c=t.append("g"),p=o*(s+a)+a;for(const l of e){const h=l.start%d*r+1,b=(l.end-l.start+1)*r-i;if(c.append("rect").attr("x",h).attr("y",p).attr("width",b).attr("height",s).attr("class","packetBlock"),c.append("text").attr("x",h+b/2).attr("y",p+s/2).attr("class","packetLabel").attr("dominant-baseline","middle").attr("text-anchor","middle").text(l.label),!k)continue;const g=l.end===l.start,f=p-2;c.append("text").attr("x",h+(g?b/2:0)).attr("y",f).attr("class","packetByte start").attr("dominant-baseline","auto").attr("text-anchor",g?"middle":"start").text(l.start),g||c.append("text").attr("x",h+b).attr("y",f).attr("class","packetByte end").attr("dominant-baseline","auto").attr("text-anchor","end").text(l.end)}},"drawWord"),K={draw:j},U={byteFontSize:"10px",startByteColor:"black",endByteColor:"black",labelColor:"black",labelFontSize:"12px",titleColor:"black",titleFontSize:"14px",blockStrokeColor:"black",blockStrokeWidth:"1",blockFillColor:"#efefef"},q=n(({packet:t}={})=>{const e=m(U,t);return`
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
	`},"styles"),at={parser:X,db:u,renderer:K,styles:q};export{at as diagram};
