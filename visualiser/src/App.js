import React from 'react';
import './App.css';
import HeatMap from '@uiw/react-heat-map';
import data from './state_file.json';

const values= []
const map = data.entries
Object.keys(map).forEach(key=> {
  let updates = map[key].updates + map[key].creations
  let date_key = new Date(key);
  values.push({date: date_key, count: updates});
})

var DISPLAY_MONTH_WINDOW = 3
var dateObj = new Date();
dateObj.setMonth(dateObj.getMonth() - DISPLAY_MONTH_WINDOW);

function App() {
  return (
    <div>
      <HeatMap
        value={values}
        weekLabels={['', 'Mon', '', 'Wed', '', 'Fri', '']}
        startDate={dateObj}
        endDate={new Date()}
      />
    </div>
  )
}

export default App;
