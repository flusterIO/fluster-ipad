import{t as G}from"./chunk-44GW5IO5-pcP672wa-BXo9xjbC.js";import{a6 as J,a5 as Q,a7 as U,a8 as Y,ao as _,an as j,n as c,v as R,u as q,aA as H,aS as K,aU as Z,aa as tt,at as et,aB as at,aV as w,aW as nt,aX as z}from"./main-BFkPyLBO.js";import{T as rt}from"./radar-VG2SY3DT-AvK8AXBj-U0yRJlsP.js";import{h as B}from"./arc-CqsYsRxe-37L-A99Q.js";import{h as it}from"./ordinal-DfAQgscy-Pc8f3NoK.js";import"./min-DAFbEQv1-BOit3uvj.js";import"./_baseUniq-C8WatXBP-C7-fz0Wb.js";import"./init-DjUOC4st-DHuO7-vr.js";function lt(t,a){return a<t?-1:a>t?1:a>=t?0:NaN}function st(t){return t}function ot(){var t=st,a=lt,l=null,m=w(0),g=w(z),S=w(0);function i(e){var n,o=(e=nt(e)).length,u,v,h=0,p=new Array(o),r=new Array(o),y=+m.apply(this,arguments),A=Math.min(z,Math.max(-z,g.apply(this,arguments)-y)),f,D=Math.min(Math.abs(A)/o,S.apply(this,arguments)),C=D*(A<0?-1:1),d;for(n=0;n<o;++n)(d=r[p[n]=n]=+t(e[n],n,e))>0&&(h+=d);for(a!=null?p.sort(function(x,T){return a(r[x],r[T])}):l!=null&&p.sort(function(x,T){return l(e[x],e[T])}),n=0,v=h?(A-o*C)/h:0;n<o;++n,y=f)u=p[n],d=r[u],f=y+(d>0?d*v:0)+C,r[u]={data:e[u],index:n,value:d,startAngle:y,endAngle:f,padAngle:D};return r}return i.value=function(e){return arguments.length?(t=typeof e=="function"?e:w(+e),i):t},i.sortValues=function(e){return arguments.length?(a=e,l=null,i):a},i.sort=function(e){return arguments.length?(l=e,a=null,i):l},i.startAngle=function(e){return arguments.length?(m=typeof e=="function"?e:w(+e),i):m},i.endAngle=function(e){return arguments.length?(g=typeof e=="function"?e:w(+e),i):g},i.padAngle=function(e){return arguments.length?(S=typeof e=="function"?e:w(+e),i):S},i}var pt=at.pie,W={sections:new Map,showData:!1},O=W.sections,F=W.showData,ct=structuredClone(pt),ut=c(()=>structuredClone(ct),"getConfig"),dt=c(()=>{O=new Map,F=W.showData,et()},"clear"),gt=c(({label:t,value:a})=>{O.has(t)||(O.set(t,a),R.debug(`added new section: ${t}, with value: ${a}`))},"addSection"),ft=c(()=>O,"getSections"),mt=c(t=>{F=t},"setShowData"),ht=c(()=>F,"getShowData"),E={getConfig:ut,clear:dt,setDiagramTitle:j,getDiagramTitle:_,setAccTitle:Y,getAccTitle:U,setAccDescription:Q,getAccDescription:J,addSection:gt,getSections:ft,setShowData:mt,getShowData:ht},yt=c((t,a)=>{G(t,a),a.setShowData(t.showData),t.sections.map(a.addSection)},"populateDb"),xt={parse:c(async t=>{const a=await rt("pie",t);R.debug(a),yt(a,E)},"parse")},wt=c(t=>`
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
`,"getStyles"),St=wt,vt=c(t=>{const a=[...t.entries()].map(l=>({label:l[0],value:l[1]})).sort((l,m)=>m.value-l.value);return ot().value(l=>l.value)(a)},"createPieArcs"),At=c((t,a,l,m)=>{R.debug(`rendering pie chart
`+t);const g=m.db,S=q(),i=H(g.getConfig(),S.pie),e=40,n=18,o=4,u=450,v=u,h=K(a),p=h.append("g");p.attr("transform","translate("+v/2+","+u/2+")");const{themeVariables:r}=S;let[y]=Z(r.pieOuterStrokeWidth);y??=2;const A=i.textPosition,f=Math.min(v,u)/2-e,D=B().innerRadius(0).outerRadius(f),C=B().innerRadius(f*A).outerRadius(f*A);p.append("circle").attr("cx",0).attr("cy",0).attr("r",f+y/2).attr("class","pieOuterCircle");const d=g.getSections(),x=vt(d),T=[r.pie1,r.pie2,r.pie3,r.pie4,r.pie5,r.pie6,r.pie7,r.pie8,r.pie9,r.pie10,r.pie11,r.pie12],$=it(T);p.selectAll("mySlices").data(x).enter().append("path").attr("d",D).attr("fill",s=>$(s.data.label)).attr("class","pieCircle");let N=0;d.forEach(s=>{N+=s}),p.selectAll("mySlices").data(x).enter().append("text").text(s=>(s.data.value/N*100).toFixed(0)+"%").attr("transform",s=>"translate("+C.centroid(s)+")").style("text-anchor","middle").attr("class","slice"),p.append("text").text(g.getDiagramTitle()).attr("x",0).attr("y",-400/2).attr("class","pieTitleText");const M=p.selectAll(".legend").data($.domain()).enter().append("g").attr("class","legend").attr("transform",(s,b)=>{const k=n+o,P=k*$.domain().length/2,I=12*n,X=b*k-P;return"translate("+I+","+X+")"});M.append("rect").attr("width",n).attr("height",n).style("fill",$).style("stroke",$),M.data(x).append("text").attr("x",n+o).attr("y",n-o).text(s=>{const{label:b,value:k}=s.data;return g.getShowData()?`${b} [${k}]`:b});const L=Math.max(...M.selectAll("text").nodes().map(s=>s?.getBoundingClientRect().width??0)),V=v+e+n+o+L;h.attr("viewBox",`0 0 ${V} ${u}`),tt(h,u,V,i.useMaxWidth)},"draw"),Tt={draw:At},Rt={parser:xt,db:E,renderer:Tt,styles:St};export{Rt as diagram};
