// Stylized Header Title
//
//
import {TitleProps } from '../interfaces/AtomProps';
import {HeaderTitleProp} from '../interfaces/MoleProps';
import Title from '../atoms/Title';

export default function HeaderTitle(props: HeaderTitleProp) {
    console.log(props.title);
    return (
        <header>
            <Title title={props.title} />
        </header>
    );
}
