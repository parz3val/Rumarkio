import { type NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import Header from "../components/Header";

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>Rumarkio : Bookmarking and Research Assistant</title>
        <meta name="description" content="Rumarkio is bookmaking application that helps you save your research" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
      <Header/> 
      </main>
    </>
  );
};

export default Home;
