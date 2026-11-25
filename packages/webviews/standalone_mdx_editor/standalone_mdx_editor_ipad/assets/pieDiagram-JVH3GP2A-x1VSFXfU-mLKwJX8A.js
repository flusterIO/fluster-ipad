import{t as _}from"./chunk-44GW5IO5-DinN5XLx-DThzNzy7.js";import{a6 as G,a5 as H,a7 as J,a8 as U,ao as X,an as Y,H as p,E as z,_ as q,aA as K,aS as Q,aU as Z,aa as tt,at as et,aB as at,aV as w,aW as nt,aX as j}from"./main-CSAj7I-T.js";import{T as rt}from"./radar-VG2SY3DT-BO1_0p1Z-xQvZ-BSh.js";import{h as R}from"./arc-Bs_HuHyE-Ddsr1_VW.js";import{h as it}from"./ordinal-DfAQgscy-Pc8f3NoK.js";import"./min-I0heXDTQ-Bmx98OAS.js";import"./_baseUniq-jNvV-1jU-CJcM8TDR.js";import"./init-DjUOC4st-DHuO7-vr.js";function lt(t,a){return a<t?-1:a>t?1:a>=t?0:NaN}function ot(t){return t}function st(){var t=ot,a=lt,l=null,h=w(0),g=w(j),S=w(0);function i(e){var n,s=(e=nt(e)).length,u,A,m=0,c=new Array(s),r=new Array(s),x=+h.apply(this,arguments),T=Math.min(j,Math.max(-j,g.apply(this,arguments)-x)),f,C=Math.min(Math.abs(T)/s,S.apply(this,arguments)),D=C*(T<0?-1:1),d;for(n=0;n<s;++n)(d=r[c[n]=n]=+t(e[n],n,e))>0&&(m+=d);for(a!=null?c.sort(function(y,$){return a(r[y],r[$])}):l!=null&&c.sort(function(y,$){return l(e[y],e[$])}),n=0,A=m?(T-s*D)/m:0;n<s;++n,x=f)u=c[n],d=r[u],f=x+(d>0?d*A:0)+D,r[u]={data:e[u],index:n,value:d,startAngle:x,endAngle:f,padAngle:C};return r}return i.value=function(e){return arguments.length?(t=typeof e=="function"?e:w(+e),i):t},i.sortValues=function(e){return arguments.length?(a=e,l=null,i):a},i.sort=function(e){return arguments.length?(l=e,a=null,i):l},i.startAngle=function(e){return arguments.length?(h=typeof e=="function"?e:w(+e),i):h},i.endAngle=function(e){return arguments.length?(g=typeof e=="function"?e:w(+e),i):g},i.padAngle=function(e){return arguments.length?(S=typeof e=="function"?e:w(+e),i):S},i}var ct=at.pie,O={sections:new Map,showData:!1},I=O.sections,W=O.showData,pt=structuredClone(ct),ut=p(()=>structuredClone(pt),"getConfig"),dt=p(()=>{I=new Map,W=O.showData,et()},"clear"),gt=p(({label:t,value:a})=>{I.has(t)||(I.set(t,a),z.debug(`added new section: ${t}, with value: ${a}`))},"addSection"),ft=p(()=>I,"getSections"),ht=p(t=>{W=t},"setShowData"),mt=p(()=>W,"getShowData"),B={getConfig:ut,clear:dt,setDiagramTitle:Y,getDiagramTitle:X,setAccTitle:U,getAccTitle:J,setAccDescription:H,getAccDescription:G,addSection:gt,getSections:ft,setShowData:ht,getShowData:mt},xt=p((t,a)=>{_(t,a),a.setShowData(t.showData),t.sections.map(a.addSection)},"populateDb"),yt={parse:p(async t=>{const a=await rt("pie",t);z.debug(a),xt(a,B)},"parse")},wt=p(t=>`
  .pieCircle{
    stroke: ${t.pieStrokeColor};
    stroke-width : ${t.pieStrokeWidth};
    opacity : ${t.pieOpacity};
  }
  .pieOuterCircle{
    stroke: ${t.pieOuterStrokeColor};
    stroke-width: ${t.pieOuterStrokeWidth};
    fill: none;
  }
  .pieTitleText {
    text-anchor: middle;
    font-size: ${t.pieTitleTextSize};
    fill: ${t.pieTitleTextColor};
    font-family: ${t.fontFamily};
  }
  .slice {
    font-family: ${t.fontFamily};
    fill: ${t.pieSectionTextColor};
    font-size:${t.pieSectionTextSize};
    // fill: white;
  }
  .legend text {
    fill: ${t.pieLegendTextColor};
    font-family: ${t.fontFamily};
    font-size: ${t.pieLegendTextSize};
  }
`,"getStyles"),St=wt,At=p(t=>{const a=[...t.entries()].map(l=>({label:l[0],value:l[1]})).sort((l,h)=>h.value-l.value);return st().value(l=>l.value)(a)},"createPieArcs"),Tt=p((t,a,l,h)=>{z.debug(`rendering pie chart
`+t);const g=h.db,S=q(),i=K(g.getConfig(),S.pie),e=40,n=18,s=4,u=450,A=u,m=Q(a),c=m.append("g");c.attr("transform","translate("+A/2+","+u/2+")");const{themeVariables:r}=S;let[x]=Z(r.pieOuterStrokeWidth);x??=2;const T=i.textPosition,f=Math.min(A,u)/2-e,C=R().innerRadius(0).outerRadius(f),D=R().innerRadius(f*T).outerRadius(f*T);c.append("circle").attr("cx",0).attr("cy",0).attr("r",f+x/2).attr("class","pieOuterCircle");const d=g.getSections(),y=At(d),$=[r.pie1,r.pie2,r.pie3,r.pie4,r.pie5,r.pie6,r.pie7,r.pie8,r.pie9,r.pie10,r.pie11,r.pie12],v=it($);c.selectAll("mySlices").data(y).enter().append("path").attr("d",C).attr("fill",o=>v(o.data.label)).attr("class","pieCircle");let E=0;d.forEach(o=>{E+=o}),c.selectAll("mySlices").data(y).enter().append("text").text(o=>(o.data.value/E*100).toFixed(0)+"%").attr("transform",o=>"translate("+D.centroid(o)+")").style("text-anchor","middle").attr("class","slice"),c.append("text").text(g.getDiagramTitle()).attr("x",0).attr("y",-400/2).attr("class","pieTitleText");const M=c.selectAll(".legend").data(v.domain()).enter().append("g").attr("class","legend").attr("transform",(o,b)=>{const k=n+s,N=k*v.domain().length/2,P=12*n,V=b*k-N;return"translate("+P+","+V+")"});M.append("rect").attr("width",n).attr("height",n).style("fill",v).style("stroke",v),M.data(y).append("text").attr("x",n+s).attr("y",n-s).text(o=>{const{label:b,value:k}=o.data;return g.getShowData()?`${b} [${k}]`:b});const L=Math.max(...M.selectAll("text").nodes().map(o=>o?.getBoundingClientRect().width??0)),F=A+e+n+s+L;m.attr("viewBox",`0 0 ${F} ${u}`),tt(m,u,F,i.useMaxWidth)},"draw"),$t={draw:Tt},zt={parser:yt,db:B,renderer:$t,styles:St};export{zt as diagram};
