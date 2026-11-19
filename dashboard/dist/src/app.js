const data = {
  total: 3.6101,
  compute: 0.3032 + 0.012633 + 0.000758,
  storage: 0.00379,
  network: 2.7288 + 0.56092,
  trend: [2.4, 3.1, 2.7, 4.4, 3.8, 4.1, 3.61],
  days: ['周一', '周二', '周三', '周四', '周五', '周六', '今天'],
};

const format = (n) => n.toFixed(n < 1 ? 4 : 3);
document.getElementById('total-emission').textContent = format(data.total);
document.getElementById('compute-emission').textContent = format(data.compute);
document.getElementById('storage-emission').textContent = format(data.storage);
document.getElementById('network-emission').textContent = format(data.network);
document.getElementById('budget-used').textContent = `${((data.total / 5) * 100).toFixed(1)}%`;

const chart = document.getElementById('trend-chart');
const max = Math.max(...data.trend);
data.trend.forEach((value, index) => {
  const group = document.createElement('div');
  group.className = 'bar-group';
  group.innerHTML = `<div class="bar" style="height:${Math.max(15, (value / max) * 165)}px"></div><small>${data.days[index]}</small>`;
  chart.appendChild(group);
});

const parts = [
  { label: '计算', value: data.compute, color: '#57e39b' },
  { label: '存储', value: data.storage, color: '#f2cc66' },
  { label: '网络', value: data.network, color: '#64b5f6' },
];
const sum = parts.reduce((acc, p) => acc + p.value, 0);
let cursor = 0;
const stops = parts.map((p) => {
  const start = cursor;
  cursor += (p.value / sum) * 360;
  return `${p.color} ${start}deg ${cursor}deg`;
});
document.getElementById('donut').style.background = `conic-gradient(${stops.join(',')})`;
document.getElementById('legend').innerHTML = parts.map((p) => `<li><span><i style="background:${p.color}"></i>${p.label}</span><b>${((p.value / sum) * 100).toFixed(1)}%</b></li>`).join('');

const hotspots = [
  { name: 'sync', value: 2.7288 },
  { name: 'upload', value: 0.5609 },
  { name: 'api_request', value: 0.3032 },
  { name: 'cache', value: 0.0126 },
];
document.getElementById('hotspots').innerHTML = hotspots.map((h) => `
  <div class="bar-row"><span>${h.name}</span><div class="track"><div class="fill" style="width:${(h.value / hotspots[0].value) * 100}%"></div></div><b>${format(h.value)} g</b></div>
`).join('');
