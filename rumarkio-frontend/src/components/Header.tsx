// Header for the application
//
import HeaderTitle from '../molecules/HeaderTitle';
import {HeaderTitleProp} from '../interfaces/MoleProps';
import Link from 'next/link';
import Logo from './Logo';


export default function Header() {
  const header_title: HeaderTitleProp = {
        title: "umarkio"
    }
  return (
    <header className="main-light-header">
    <ul>
    <Link href="#"><span>{"$-<"}</span></Link>
    <Link href="#"><span><Logo></Logo><HeaderTitle title={header_title.title}/></span></Link>
    </ul>
    </header>
  );
}
