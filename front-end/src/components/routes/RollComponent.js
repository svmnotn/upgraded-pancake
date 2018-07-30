import React from 'react';
import '../../styles/App.css';

import Table from './components/Table';
class Roll extends React.Component {
    render() {
        return(
            <div>
                <Table title={this.props.location.state.title}
                       heading={this.props.location.state.heading}
                       dice={this.props.location.state.dice}
                       results={this.props.location.state.results}
                       urlData={this.props.location.state.urlData}/>
            </div>
        )
    }
}

export default Roll;
