import * as css from 'csstype';

const global_styles = {
    global_center: {
        position: 'fixed',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
    } as css.Properties,
}

export {
    global_styles as styles,
}